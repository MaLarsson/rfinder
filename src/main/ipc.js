//
// Registers the ipc calls.
//

const register = () => {
  const { ipcMain } = require('electron')
  const addon = require('../../native/index')

  addon.hello()
  console.log("register!")
}

exports.register = register
