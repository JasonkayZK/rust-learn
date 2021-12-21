import { useState, useEffect } from 'react'

export default function Settings() {
  const [authToken, setAuthToken] = useState(() => {
    return window.localStorage.getItem('authorization') || ''
  })

  useEffect(() => {
    window.localStorage.setItem('authorization', authToken)
  })

  return (
    <fieldset>
      <legend>Authorization</legend>
      <input
        type="text"
        value={authToken}
        size={50}
        onChange={(e) => setAuthToken(e.target.value)}
      />
      <button className="pure-button pure-button-primary">Save</button>
    </fieldset>
  )
}
