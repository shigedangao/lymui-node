const fs = require('fs')
const path = require('path')
const request = require('request')
const unzipper = require('unzipper')
const chalk = require('chalk')

// Global variable
const LIB_VERSION = '1.2.2'
const binPath = 'lymui'

/**
 * Installer
 *    Install the liblymui C lib in the bin folder
 * 
 * @void
 */
const installer = () => {
  let os
  const platform = process.platform.toLowerCase()

  if (platform === 'darwin') {
    os = 'osx'
  } else {
    os = 'linux'
  }

  const libTag = `liblymui-${os}-${LIB_VERSION}.zip`
  const link = `https://github.com/MarcInthaamnouay/lymui/releases/download/v${LIB_VERSION}-${os}/${libTag}`

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
