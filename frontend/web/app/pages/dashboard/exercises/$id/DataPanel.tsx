import { AreaChart } from '@mantine/charts';
import { Loader } from "./route";
import { Group, Stack, Title } from '@mantine/core';
import { DatesProvider, MonthPickerInput, DatePickerInput } from '@mantine/dates';
import { useState } from 'react';

export const data = [
  {
    date: 'Mar 22',
    Apples: 2890,
    Oranges: 2338,
    Tomatoes: 2452,
  },
  {
    date: 'Mar 23',
    Apples: 2756,
    Oranges: 2103,
    Tomatoes: 2402,
  },
  {
    date: 'Mar 24',
    Apples: 3322,
    Oranges: 986,
    Tomatoes: 1821,
  },
  {
    date: 'Mar 25',
    Apples: 3470,
    Oranges: 2108,
    Tomatoes: 2809,
  },
  {
    date: 'Mar 26',
    Apples: 3129,
    Oranges: 1726,
    Tomatoes: 2290,
  },
]

type DataPanelProps = {
  rootData: Loader
}

export default function DataPanel({
  rootData
}: DataPanelProps) {
  const [from, setFrom] = useState<Date | null>(null);
  const [to, setTo] = useState<Date | null>(null);

  return <Stack gap="xl">
    <Title order={2}>Key Interest Points</Title>
    <AreaChart
      h="300"
      data={data}
      dataKey="date"
      series={[
        { name: 'Apples', color: 'indigo.6' },
        { name: 'Oranges', color: 'blue.6' },
        { name: 'Tomatoes', color: 'teal.6' },
      ]}
      curveType="linear"
      withLegend
      legendProps={{ verticalAlign: 'bottom' }}
    />
    <Title order={2}>Pose Detection</Title>
    <AreaChart
      h={300}
      data={data}
      dataKey="date"
      series={[
        { name: 'Apples', color: 'indigo.6' },
        { name: 'Oranges', color: 'blue.6' },
        { name: 'Tomatoes', color: 'teal.6' },
      ]}
      curveType="linear"
      withLegend
      legendProps={{ verticalAlign: 'bottom' }}
    />
  </Stack>;
}
