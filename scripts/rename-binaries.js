/**
 * This script is used to rename the binary with the platform specific postfix.
 * When `tauri build` is ran, it looks for the binary name appended with the platform specific postfix.
 */

import {execa} from 'execa';
import { existsSync, renameSync } from 'fs'

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
  if (existsSync(`src-tauri/bin/extractor${extension}`)) {
    renameSync(
      `src-tauri/bin/extractor${extension}`,
      `src-tauri/bin/extractor-${targetTriple}${extension}`
    )
  }
  if (existsSync(`src-tauri/bin/gk${extension}`)) {
    renameSync(
      `src-tauri/bin/gk${extension}`,
      `src-tauri/bin/gk-${targetTriple}${extension}`
    )
  }
  if (existsSync(`src-tauri/bin/goalc${extension}`)) {
    renameSync(
      `src-tauri/bin/goalc${extension}`,
      `src-tauri/bin/goalc-${targetTriple}${extension}`
    )
  }
}

main().catch((e) => {
  throw e
})
