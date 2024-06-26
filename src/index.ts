import { Command } from 'commander'
import packageJSON from '../package.json' assert { type: 'json' }

const rc = new Command()

rc
  .name('rc')
  .usage('<command> [options]')
  .description('Source code management')
  .version(packageJSON.version, '-v, --version', 'output the current version')
  .helpOption('-h, --help', 'Display help for command')
  .allowUnknownOption()

rc.parse()
