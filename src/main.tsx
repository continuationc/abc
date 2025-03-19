import Model from 'react-dom/client'
import React from 'react'
import Index from './lib'

Model.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <Index />
  </React.StrictMode>,
)
