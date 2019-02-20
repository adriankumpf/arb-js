const arb = require('../native')

/**
 * Given an array of ids activates the corresponding relays. An empty array turns off all relays.
 *
 * The relays are labeled from 1 to 8 according to the
 * [data sheet](http://www.abacom-online.de/div/ABACOM_USB_LRB.pdf).
 *
 * @param    {Array} relays             ids of relays to activate
 * @param   {Number} [opts.port]        which USB port to use
 * @param  {Boolean} [opts.verify=true] whether the activation should be verfied
 *
 * @throws   {Error}                    if activating the relay(s) fails
 */
module.exports.activate =  function(relays, opts) {
  if (!relays || !Array.isArray(relays)) throw Error('relays is not an array')
  if (opts === null || typeof opts !== 'object') opts = {}

  const port = opts.port || null
  const verify = opts.verify !== false

  return arb.activate(relays, verify, port)
}
