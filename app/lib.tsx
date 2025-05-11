import { StrictMode } from 'react'
import { RouterProvider, Outlet, Link, createBrowserRouter } from 'react-router'
import { MantineProvider, AppShell, NavLink, createTheme } from '@mantine/core'
import { Group, Avatar, UnstyledButton } from '@mantine/core'
import { Home, Help, Page } from './page'
import '@mantine/core/styles.css'

const Index = () => (
  <AppShell
    header={{ height: { base: 60, md: 60, lg: 60 } }}
    navbar={{ width: { base: 200, md: 300, lg: 400 }, breakpoint: 'sm' }}
    padding="md"
  >
    <AppShell.Header>
      <Group h="100%" px="xl">
        <Group justify="space-between" style={{ flex: 1 }}>
          <Group>
            <Avatar color="gray" variant="filled">
              AB
            </Avatar>
          </Group>
          <Group>
            <UnstyledButton component={Link} to="/">
              Home
            </UnstyledButton>
            <UnstyledButton component={Link} to="/help">
              Help
            </UnstyledButton>
          </Group>
        </Group>
      </Group>
    </AppShell.Header>
    <AppShell.Navbar p="md">
      <NavLink component={Link} to="/page" label="Page"></NavLink>
      <NavLink component={Link} to="/page" label="Page"></NavLink>
    </AppShell.Navbar>
    <AppShell.Main>
      <Outlet />
    </AppShell.Main>
  </AppShell>
)

const index = createBrowserRouter([
  {
    Component: Index,
    children: [
      { index: true, Component: Home },
      { path: 'help', Component: Help },
      { path: 'page', Component: Page },
    ],
  },
])

const theme = createTheme({
  primaryColor: 'dark',
})

const App = () => (
  <StrictMode>
    <MantineProvider theme={theme}>
      <RouterProvider router={index} />
    </MantineProvider>
  </StrictMode>
)

export default App
