import { Table } from "@mantine/core";

type Column = {
  key: string;
  title: string;
}

type CustomTableProps<T> = {
  columns: Column[];
  data: T[];
}

export default function CustomTable<T>({
  columns,
  data,
}: CustomTableProps<T>) {

  return (
    <Table>
      <Table.Thead>
        <Table.Tr>
          {columns.map((column) => (
            <Table.Th key={column.key}>{column.title}</Table.Th>
          ))}
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>
        {data.map((row) => (
          <Table.Tr>
            {columns.map((column) => (
              <Table.Td key={column.key}>
                {/* @ts-ignore */}
                {row[column.key]}
              </Table.Td>
            ))}
          </Table.Tr>
        ))}
      </Table.Tbody>
    </Table>
  );
}