import * as log from '@tauri-apps/plugin-log'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useLocation } from 'react-router'
import { useState, useEffect } from 'react'
import { Code, Text, Group, Stack, Button, TextInput } from '@mantine/core'

export const Home = () => {
  let input = useState({})
  let state = useState({})
  let fresh = () => invoke<{}>('fresh')
  let start = () => invoke<{}>('start', { model: input[0] })
  let close = () => invoke<{}>('close', { model: input[0] })

  useEffect(() => {
    listen<{}>('state', ({ payload: value }) => state[1](value))
  }, [])

  useEffect(() => {
    fresh()
  }, [])

  return (
    <>
      <Stack>
        <Text>Welcome</Text>
        <Group>
          <Button onClick={() => start()}>Start</Button>
          <Button onClick={() => close()}>Close</Button>
          <Button onClick={() => fresh()}>Fresh</Button>
        </Group>
        <Group>
          <TextInput
            label="Input"
            value={JSON.stringify(input[0], null, 2)}
            onChange={({ currentTarget: { value } }) =>
              input[1](JSON.parse(value))
            }
          />
        </Group>
        <Code block>{JSON.stringify(state[0], null, 2)}</Code>
      </Stack>
    </>
  )
}

export const Help = () => {
  return <>Help</>
}

export const Page = () => {
  let location = useLocation()
  return <>Page {location.pathname}</>
}
