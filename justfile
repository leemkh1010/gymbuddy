default:
  just --list

gen-proto: clean
  cd proto && \
  buf generate && \
  mv web/dist/* ../backend/types/proto

clean:
  rm -rf sdk/out/*

# cqlsh -u cassandra -p cassandra
# create user if not exists 'admin' with password 'local' nosuperuser;
# grant all permissions on keyspaces exercise_analyser to admin;