import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Register from './components/Register';
import Login from './components/Login';

const App = () => {
  return (
      <Router>
        <div className="App">
          <Switch>
            <Route path="/register" component={Register} />
            <Route path="/login" component={Login} />
            {/* Add more routes for other components/pages */}
          </Switch>
        </div>
      </Router>
  );
};

export default App;
