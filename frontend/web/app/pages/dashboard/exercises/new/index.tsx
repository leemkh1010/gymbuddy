'use client';

import { Anchor, Button, Container, Group, Progress, rem, Select, Stack, Text, TextInput } from "@mantine/core";
import { ActionFunctionArgs, LoaderFunctionArgs } from "react-router";
import { Form, UIMatch } from "react-router";
import { Handle } from "~/utils/hook";
import { capitalise } from "~/utils/utils";
import { Dropzone } from '@mantine/dropzone';
import { useListState } from "@mantine/hooks";
import { IconUpload, IconX, IconPhoto } from "@tabler/icons-react";
import { useState } from "react";
import { DateInput, DatePicker } from "@mantine/dates";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>New</Anchor>;
  }
}

enum Exercises {
  SQUAT = "SQUAT",
}

enum CameraView {
  FRONT = "FRONT",
  LEFT_SIDE = "LEFT_SIDE",
  RIGHT_SIDE = "RIGHT_SIDE",
  BACK = "BACK",
}

export const loader = async ({

}: LoaderFunctionArgs) => {
  return {};
}

export const action = async ({
  request
}: ActionFunctionArgs) => {
  const formData = await request.formData();
  console.log(formData.values());
  return null;
}

type Video = {
  uploadUrl: string;
  uploadPercent: number;
  done: boolean;
  file: File;
};

export default function ExercisesNew() {
  const [videos, setVideos] = useListState<Video>([]);

  const onDrop = (files: File[]) => {
    const newVideos = files.map((file) => {
      return {
        uploadUrl: '',
        uploadPercent: 0,
        done: false,
        file,
      } as Video;
    });

    setVideos.append(...newVideos);
  };

  const onCreateClick = () => {

  };

  return (
    <Container w="100%" maw="1080px">
      <Form method="post" action="/dashboard/exercises/new">
        <Stack gap="md">
          <TextInput
            required
            label="Name"
          />
          <TextInput
            label="Description"
          />
          <Select
            required
            label="Type"
            data={Object.keys(Exercises).map((key) => ({ value: key, label: capitalise(key) }))}
          />
          <Select
            required
            label="Client"
            data={Object.keys(CameraView).map((key) => ({ value: key, label: capitalise(key) }))}
          />
          <Select
            required
            label="Trainer"
            data={Object.keys(CameraView).map((key) => ({ value: key, label: capitalise(key) }))}
          />
          <DateInput 
            label="Date"
            clearable
            defaultDate={new Date()}
            description="Optional: Default to created time"
          />
          <Dropzone
            onDrop={onDrop}
            accept={[
              'video/mp4',
              'video/quicktime',
            ]}
          >
            <Group justify="center" gap="xl" mih={220} style={{ pointerEvents: 'none' }}>
              <Dropzone.Accept>
                <IconUpload
                  style={{ width: rem(52), height: rem(52), color: 'var(--mantine-color-blue-6)' }}
                  stroke={1.5}
                />
              </Dropzone.Accept>
              <Dropzone.Reject>
                <IconX
                  style={{ width: rem(52), height: rem(52), color: 'var(--mantine-color-red-6)' }}
                  stroke={1.5}
                />
              </Dropzone.Reject>
              <Dropzone.Idle>
                <IconPhoto
                  style={{ width: rem(52), height: rem(52), color: 'var(--mantine-color-dimmed)' }}
                  stroke={1.5}
                />
              </Dropzone.Idle>

              <div>
                <Text size="xl" inline>
                  Drag Video here or click to select files
                </Text>
                <Text size="sm" c="dimmed" inline mt={7}>
                  Only mp4 / mov files are supported
                </Text>
              </div>
            </Group>
          </Dropzone>
          <Stack gap="md">
            {videos.length ? videos.map((video, index) => {
              return (
                <VideoUpload
                  key={index}
                  index={index}
                  video={video}
                  setVideos={setVideos}
                />
              );
            }) : null}
          </Stack>
          <Button disabled={
            !videos.length || !!(videos.length && !videos.every((video) => video.done))
          } onClick={onCreateClick} type="submit">
            {!videos.length ? 'Please add video claps' : videos.every((video) => video.done) ? 'Create' : 'Upload Video'}
          </Button>
        </Stack>
      </Form>
    </Container>
  );
}

type VideoUploadProps = {
  index: number;
  video: Video;
  setVideos: ReturnType<typeof useListState<Video>>[1];
};

const VideoUpload = ({ index, video, setVideos }: VideoUploadProps) => {
  console.log(video.file)
  return (
    <Stack gap="md">
      <Group align="center" justify="space-between">
        <video width={320} height={240} controls>
          <source src={URL.createObjectURL(video.file)} type={video.file.type} />
        </video>
        <Stack gap="md" justify="space-around">
          <Text>{video.file.name}</Text>
          <Select
            label="Camera View"
            data={Object.keys(CameraView).map((key) => ({ value: key, label: capitalise(key) }))}
          />
          <Button
            onClick={() => setVideos.remove(index)}
            variant="link"
            color="red"
          >
            Remove
          </Button>
        </Stack>
      </Group>
      <Progress value={video.uploadPercent} />
    </Stack>
  )
}