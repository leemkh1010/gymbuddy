use anyhow::Result;
use scylla::{
    batch::Batch, frame::value::CqlTimestamp, query::{self, Query}, transport::session::{CurrentDeserializationApi, GenericSession}, SessionBuilder
};

use crate::models::{Client, ClientsByEmailRow, ClientsByIdRow, Trainer};

pub type CassandraSession = GenericSession<CurrentDeserializationApi>;

pub struct Cassandra {
    instance: CassandraSession,
}

pub trait CoreRepo {
    // Client
    async fn get_clients(&self, limit: i32) -> Result<Vec<Client>>;
    async fn get_client_by_id(&self, id: &String) -> Result<Client>;
    async fn get_client_by_email(&self, email: &String) -> Result<Client>;
    async fn create_client(&self, client: &Client) -> Result<Client>;
    async fn update_client(&self, client: &Client) -> Result<Client>;
    async fn delete_client(&self, id: &String) -> Result<()>;

    // Trainer
    async fn get_trainers(&self) -> Result<Vec<Trainer>>;
    async fn get_trainer(&self, id: &String) -> Result<Trainer>;
    async fn create_trainer(&self, trainer: Trainer) -> Result<Trainer>;
    async fn update_trainer(&self, trainer: Trainer) -> Result<Trainer>;
    async fn delete_trainer(&self, id: &String) -> Result<()>;

    // Organiser

    // Exercise

    // Media
}

pub enum Tables {
    OrganisersById,
    OrganisersLoginByEmailPassword,
    ClientsById,
    ClientsByEmail,
    TrainersById,
    TrainersByEmail,
    ExercisesById,
}

impl Cassandra {
    pub async fn new(host: &str, port: u16, db: &str) -> Self {
        let cassandra = SessionBuilder::new()
            .use_keyspace(db, false)
            .user("admin", "local")
            .known_nodes([format!("{host}:{port}")])
            .build()
            .await;

        let session = match cassandra {
            Ok(session) => Cassandra { instance: session },
            Err(e) => panic!("Error connecting to Cassandra: {:?}", e),
        };

        session
    }

    pub fn get_instance(&self) -> &CassandraSession {
        &self.instance
    }

    pub fn get_keyspace(&self) -> String {
        self.instance.get_keyspace().unwrap().to_string().to_owned()
    }

    pub fn get_table(&self, table: Tables) -> String {
        let keyspace = self.get_keyspace();
        match table {
            Tables::ClientsById => format!("{:?}.clients_by_id", keyspace),
            Tables::ClientsByEmail => format!("{:?}.clients_by_email", keyspace),
            Tables::ExercisesById => format!("{:?}.exercises_by_id", keyspace),
            Tables::TrainersById => format!("{:?}.trainers_by_id", keyspace),
            Tables::TrainersByEmail => format!("{:?}.trainers_by_email", keyspace),
            Tables::OrganisersById => format!("{:?}.organisers_by_id", keyspace),
            Tables::OrganisersLoginByEmailPassword => {
                format!("{:?}.organisers_login_by_email_password", keyspace)
            }
        }
    }

    pub fn get_timestamp(&self, ms: i64) -> CqlTimestamp {
        CqlTimestamp(ms)
    }

    pub fn select_all_query(&self, table: Tables) -> Query {
        let table = self.get_table(table);
        Query::new(format!("SELECT * FROM {table}"))
    }

    pub fn select_by_id_query(&self, table: Tables) -> Query {
        let table = self.get_table(table);
        Query::new(format!("SELECT * FROM {table} WHERE id = ?"))
    }

    pub fn select_by_email_query(&self, table: Tables) -> Query {
        let table = self.get_table(table);
        Query::new(format!("SELECT * FROM {table} WHERE email = ?"))
    }

    pub fn delete_by_id_query(&self, table: Tables) -> Query {
        let table = self.get_table(table);
        Query::new(format!("DELETE FROM {table} WHERE id = ?"))
    }

    pub fn delete_by_email_query(&self, table: Tables) -> Query {
        let table = self.get_table(table);
        Query::new(format!("DELETE FROM {table} WHERE email = ?"))
    }
}

impl CoreRepo for Cassandra {
    // Client
    async fn get_clients(&self, limit: i32) -> Result<Vec<Client>> {
        let db_instance = self.get_instance();
        let query = self.select_all_query(Tables::ClientsById)
            .with_page_size(limit);

        let res = db_instance.query_unpaged(query, &[]).await;

        if res.is_err() {
            return Err(res.unwrap_err().into());
        }

        let rows = res.unwrap().into_rows_result()?;
        
        let rows = rows.rows::<ClientsByIdRow>()?;

        let clients: Vec<Client> = rows.map(|r| {
            let row = r.unwrap();
            Client {
                id: row.id,
                first_name: row.first_name,
                last_name: row.last_name,
                email: row.email,
                created_at: Some(row.created_at.0),
                updated_at: Some(row.updated_at.0),
            }
        }).collect();

        Ok(clients)
    }

    async fn create_client(&self, client: &Client) -> Result<Client> {
        let db_instance = self.get_instance();
        let mut batch = Batch::default();

        let id_table = self.get_table(Tables::ClientsById);
        let query = Query::new(
            format!("INSERT INTO {id_table} (id, first_name, last_name, email, created_at, updated_at) VALUES(?, ?, ?, ?, ?, ?)")
        );
        batch.append_statement(query);

        let email_table = self.get_table(Tables::ClientsByEmail);
        let query = Query::new(
            format!("INSERT INTO {email_table} (client_id, email) VALUES(?, ?)")
        );
        batch.append_statement(query);

        let batch_values = (
            (
                &client.id,
                &client.first_name,
                &client.last_name,
                &client.email,
                CqlTimestamp(client.created_at.unwrap()),
                CqlTimestamp(client.updated_at.unwrap()),
            ),
            (&client.id, &client.email),
        );

        db_instance.batch(&batch, batch_values).await.unwrap();

        Ok(client.clone())
    }

    async fn get_client_by_id(&self, id: &String) -> Result<Client> {
        let db_instance = self.get_instance();

        let query = self.select_by_id_query(Tables::ClientsById);

        let req = db_instance.query_unpaged(query, (&id,)).await;

        let row = req.unwrap().into_rows_result()?.first_row::<ClientsByIdRow>();

        match row {
            Ok(row) => {
                Ok(Client {
                    id: row.id,
                    first_name: row.first_name,
                    last_name: row.last_name,
                    email: row.email,
                    created_at: Some(row.created_at.0),
                    updated_at: Some(row.updated_at.0),
                })
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e.into())
            }
        }
    }

    async fn get_client_by_email(&self, email: &String) -> Result<Client> {
        let db_instance = self.get_instance();
        let query = self.select_by_email_query(Tables::ClientsByEmail);

        let req = db_instance.query_unpaged(query, (&email,)).await;

        let row = req.unwrap().into_rows_result()?.first_row::<ClientsByEmailRow>();

        match row {
            Ok(row) => {
                let client = self.get_client_by_id(&row.client_id).await.unwrap();
                Ok(client)
            },
            Err(e) => {
                println!("Error: {:?}", e);
                Err(e.into())
            }
        }
    }

    async fn update_client(&self, client: &Client) -> Result<Client> {
        let db_instance = self.get_instance();
        let table = self.get_table(Tables::ClientsById);
        let query = Query::new(
            format!("UPDATE {table} SET first_name = ?, last_name = ?, updated_at = ? WHERE id = ? and created_at = ?")
        );

        db_instance
            .query_unpaged(
                query,
                (
                    &client.first_name,
                    &client.last_name,
                    self.get_timestamp(client.updated_at.unwrap()),
                    &client.id,
                    self.get_timestamp(client.created_at.unwrap()),
                ),
            )
            .await
            .unwrap();

        Ok(client.clone())
    }

    async fn delete_client(&self, id: &String) -> Result<()> {
        let db_instance = self.get_instance();
        let mut batch = Batch::default();
        let id_query = self.delete_by_id_query(Tables::ClientsById);
        let email_query = self.delete_by_email_query(Tables::ClientsByEmail);

        batch.append_statement(id_query);
        batch.append_statement(email_query);

        let batch_values = ((id), (id));

        // db_instance.batch(&batch, batch_values).await.unwrap();

        Ok(())
    }

    // Trainer
    async fn get_trainers(&self) -> Result<Vec<Trainer>> {
        Ok(vec![])
    }

    async fn create_trainer(&self, trainer: Trainer) -> Result<Trainer> {
        Ok(trainer)
    }

    async fn get_trainer(&self, id: &String) -> Result<Trainer> {
        Ok(Trainer {
            id: id.clone(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "".to_string(),
            created_at: None,
            updated_at: None,
        })
    }

    async fn update_trainer(&self, trainer: Trainer) -> Result<Trainer> {
        Ok(trainer)
    }

    async fn delete_trainer(&self, id: &String) -> Result<()> {
        Ok(())
    }
}
