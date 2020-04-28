import React, { Component } from 'react'
import ReactDOM from 'react-dom'

// TODO move to component
class App extends Component {
  render() {
    return (
      <div>
	<h1>My React App</h1>
      </div>
    );
  }
}

ReactDOM.render(<App />, document.getElementById('app'))
