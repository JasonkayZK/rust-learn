import { useState, useEffect } from 'react'
import { useParams, useHistory } from 'react-router-dom'
import { getURLMap, updateURLMap } from '../api/url_map'

export default function URLMapEdit() {
  const { id } = useParams<{id: string}>()

  const history = useHistory()
  const [key, setKey] = useState(id)
  const [url, setURL] = useState('')

  useEffect(() => {
    getURLMap(key).then((urlMap) => {
      setKey(urlMap.key)
      setURL(urlMap.url)
    })
  }, [key])

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    await updateURLMap({key: key, url: url})
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

      <button type='submit' className='pure-button pure-button-primary'>Save</button>
      <button className='pure-button' onClick={handleCancel}>Cancel</button>
    </form>
  )
}
