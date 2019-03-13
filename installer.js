const fs = require('fs')
const path = require('path')
const request = require('request')
const unzipper = require('unzipper')
const chalk = require('chalk')

// Global variable
const LIB_VERSION = '1.3.0'
const binPath = 'lymui'

/**
 * Installer
 *    Install the liblymui C lib in the bin folder
 * 
 * @void
 */
const installer = () => {
  const platform = process.platform
  const libTag = `liblymui-${platform}-${LIB_VERSION}.zip`
  const link = `https://github.com/MarcInthaamnouay/lymui/releases/download/v${LIB_VERSION}/${libTag}`

  console.warn(chalk.blueBright(`Downloading liblymui ${LIB_VERSION}.`))

  request(link)
    .pipe(fs.createWriteStream(libTag))
    .on('close', () => {
      fs.createReadStream(libTag)
        .pipe(unzipper.Extract({ path: path.resolve(__dirname, binPath) }))
        .on('close', () => console.warn(chalk.blueBright('Download & installation done.')))
        .on('error', err => console.warn(chalk.red(`error happened during download: ${err}`)))
    })
}

installer()
