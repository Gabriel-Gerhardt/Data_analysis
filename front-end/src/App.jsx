import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './index.css'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100">
      <div className="flex space-x-4">
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo w-24 h-24" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react w-24 h-24" alt="React logo" />
        </a>
      </div>
      <h1 className="text-4xl font-bold text-gray-800 mt-8">Vite + React</h1>
      <div className="card p-6 bg-white rounded-lg shadow-lg mt-4">
        <button
          className="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-700"
          onClick={() => setCount(count + 1)}
        >
          count is {count}
        </button>
        <p className="mt-2">
          <code className="text-blue-500"></code> 
        </p>
      </div>
    </div>
  )
}

export default App;
