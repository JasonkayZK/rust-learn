import React from 'react'
import {
  BrowserRouter as Router,
  Switch,
  Route,
  NavLink
} from 'react-router-dom'
import './App.css'

import Settings from './Settings'
import URLMapNew from './url_maps/URLMapNew'
import URLMapEdit from './url_maps/URLMapEdit'
import URLMapIndex from './url_maps/URLMapIndex'

function App() {
  return (
    <Router>
      <div className="pure-menu pure-menu-horizontal">
        <ul className="pure-menu-items">
          <li className="pure-menu-item">
            <NavLink to="/" className="pure-menu-link">Home</NavLink>
          </li>
          <li className="pure-menu-item">
            <NavLink to="/settings" className="pure-menu-link">Settings</NavLink>
          </li>
        </ul>
      </div>

      <Switch>
        <Route path="/settings">
          <Settings />
        </Route>
        <Route path="/url_maps/new">
          <URLMapNew />
        </Route>
        <Route path="/url_maps/edit/:id">
          <URLMapEdit />
        </Route>
        <Route path="/">
          <URLMapIndex />
        </Route>
      </Switch>
    </Router>
  )
}

export default App
