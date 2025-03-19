import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import logo from './assets/react.svg'
import './lib.css'

const greet = (input: string) => invoke<string>('greet', { input })

export const App = () => {
  const [input, setInput] = useState('')
  const [reply, setReply] = useState('')

  return (
    <main className="container">
      <h1>Welcome</h1>

      <div className="row">
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={logo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click</p>

      <form
        className="row"
        onSubmit={async (e) => {
          e.preventDefault()
          setReply(await greet(input))
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setInput(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{reply}</p>
    </main>
  )
}

export default App
