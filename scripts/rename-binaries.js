/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */

const execa = require('execa')
const fs = require('fs')

let extension = ''
if (process.platform === 'win32') {
  extension = '.exe'
}

async function main() {
  const rustInfo = (await execa('rustc', ['-vV'])).stdout
  const targetTriple = /host: (\S+)/g.exec(rustInfo)[1]
  if (!targetTriple) {
    console.error('Failed to determine platform target triple')
  }
  if (fs.existsSync(`src-tauri/bin/extractor${extension}`)) {
    fs.renameSync(
      `src-tauri/bin/extractor${extension}`,
      `src-tauri/bin/extractor-${targetTriple}${extension}`
    )
  }
  if (fs.existsSync(`src-tauri/bin/gk${extension}`)) {
    fs.renameSync(
      `src-tauri/bin/gk${extension}`,
      `src-tauri/bin/gk-${targetTriple}${extension}`
    )
  }
  if (fs.existsSync(`src-tauri/bin/goalc${extension}`)) {
    fs.renameSync(
      `src-tauri/bin/goalc${extension}`,
      `src-tauri/bin/goalc-${targetTriple}${extension}`
    )
  }
}

main().catch((e) => {
  throw e
})
