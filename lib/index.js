var addon = require('../native')

function activate (relays, opts) {
  if (!relays || !Array.isArray(relays)) throw Error('relays is not an array')
  if (opts === null || typeof opts !== 'object') opts = {}

  const port = opts.port || 0
  const verify = opts.verify || true

  return addon.activate(relays, verify, port)
}

module.exports = {
  activate
}
