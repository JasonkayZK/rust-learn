import { useState, useEffect } from 'react'
import { Link, useHistory } from 'react-router-dom'
import { API_HOST, URLMap, getURLMaps, deleteURLMap } from '../api/url_map'

interface URLMapProp {
  keyword: string
  url: string
}

const URLMapRow = (urlMap: URLMapProp) => {
  const history = useHistory()
  const handleDelete = (e: React.FormEvent) => {
    if (window.confirm("Are you sure?")) {
      e.preventDefault()
      deleteURLMap(urlMap.keyword).then(() => {
        history.go(0)
      })
    }
  }
  return (
    <tr>
      <td>{urlMap.keyword}</td>
      <td>{urlMap.url}</td>
      <td>
        <a href={`${API_HOST}/${urlMap.keyword}`} target='_blank' rel='noreferrer noopener'>
          Test
        </a>
        <Link to={`/url_maps/edit/${urlMap.keyword}`}>Edit</Link>
        <a href='/#' onClick={handleDelete}>Delete</a>
      </td>
    </tr>
  )
}

const URLMapEmptyRow = () => {
  return (
    <tr>
      <td colSpan={3}>No URLMaps!</td>
    </tr>
  )
}

export default function URLMapIndex() {
  const [urlMaps, setURLMaps] = useState(Array<URLMap>())

  useEffect(() => {
    getURLMaps().then(setURLMaps)
  }, [])

  let rows = urlMaps.map((urlMap) => <URLMapRow keyword={urlMap.key} {...urlMap} />)
  if (rows.length === 0) {
    rows = [
      <URLMapEmptyRow key={0} />
    ]
  }

  return (
    <table className='pure-table pure-table-striped'>
      <thead>
        <tr>
          <th>Key</th>
          <th>URL</th>
          <th>Actions <Link to='/url_maps/new'>Create</Link></th>
        </tr>
      </thead>
      <tbody>
        { rows }
      </tbody>
    </table>
  )
}
