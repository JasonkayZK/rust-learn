import { useState } from 'react'
import { useHistory } from 'react-router-dom'
import { createURLMap } from '../api/url_map'

export default function URLMapNew() {
  const history = useHistory()
  const [key, setKey] = useState('')
  const [url, setURL] = useState('')

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    await createURLMap({key: key, url: url})
    history.push('/')
  }

  const handleCancel = (e: React.FormEvent) => {
    e.preventDefault()
    history.goBack()
  }

  return (
    <form className="pure-form pure-form-stacked" onSubmit={handleSubmit}>
      <label>Key</label>
      <input type='text' value={key} onChange={(e) => setKey(e.target.value)} />

      <label>URL</label>
      <input type='text' value={url} className='pure-input-1' onChange={(e) => setURL(e.target.value)} />

      <button type='submit' className='pure-button pure-button-primary'>Create</button>
      <button className='pure-button' onClick={handleCancel}>Cancel</button>
    </form>
  )
}
