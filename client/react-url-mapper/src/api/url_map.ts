export const API_HOST=process.env.REACT_APP_API_HOST

export interface URLMap {
  key: string
  url: string
}

const buildHeaders = () => {
  const headers = new Headers()
  headers.set('Content-Type', 'application/json')
  const authToken = window.localStorage.getItem('authorization') || ''
  headers.set('Authorization', authToken)
  return headers
}

export const getURLMaps = async () => {
  const response = await fetch(`${API_HOST}/api/url_maps`, {
    headers: buildHeaders()
  })
  let urlMaps: Array<URLMap> = await response.json()
  return urlMaps
}

export const getURLMap = async (key: string) => {
  const response = await fetch(`${API_HOST}/api/url_maps/${key}`, {
    headers: buildHeaders()
  })
  let urlMap: URLMap = await response.json()
  return urlMap
}

export const createURLMap = async (urlMap: URLMap) => {
  const response = await fetch(`${API_HOST}/api/url_maps`, {
    method: 'POST',
    headers: buildHeaders(),
    body: JSON.stringify(urlMap)
  })
  let createdURLMap: URLMap = await response.json()
  return createdURLMap
}

export const updateURLMap = async (urlMap: URLMap) => {
  const response = await fetch(`${API_HOST}/api/url_maps/${urlMap.key}`, {
    method: 'PUT',
    headers: buildHeaders(),
    body: JSON.stringify(urlMap)
  })
  let updatedURLMap: URLMap = await response.json()
  return updatedURLMap
}

export const deleteURLMap = async (key: string) => {
  const response = await fetch(`${API_HOST}/api/url_maps/${key}`, {
    method: 'DELETE',
    headers: buildHeaders(),
  })
  return response.ok
}
