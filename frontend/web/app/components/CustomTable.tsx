import { Table } from "@mantine/core";

type Column = {
  key: string;
  title: string;
}

type CustomTableProps<T> = {
  columns: Column[];
  data: T[];
  children: React.ReactNode;
}

export default function CustomTable<T>({
  columns,
  data,
  children,
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
        {children}
      </Table.Tbody>
    </Table>
  );
}