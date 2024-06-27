import process from 'node:process'
import { Command } from 'commander'
import consola from 'consola'
import chalk from 'chalk'
import packageJSON from '../package.json' assert { type: 'json' }
import { findMostMatchText } from './helpers/diff'

const rc = new Command()

export type CommandName = 'init'

const commandList: CommandName[] = ['init']

rc
  .name('rc')
  .usage('<command> [options]')
  .description(chalk.bold.gray('Simple REST Client is an easy-to-use command-line tool for interacting with RESTful APIs.'))
  .version(packageJSON.version, chalk.blue('-v, --version', 'output the current version'))
  .helpOption(chalk.blue('-h, --help', 'Display help for command'))
  .allowUnknownOption()
  .action((_, command) => {
    let isArgs = false

    if (command) {
      const args = command.args?.[0]

      if (args && !commandList.includes(args as CommandName)) {
        isArgs = true
        const matchCommand = findMostMatchText(commandList, args)

        if (!matchCommand) {
          consola.error(`Unknown command '${args}'`)
          return
        }

        consola.warn(`Unknown command '${args}', Did you mean '${chalk.underline(matchCommand)}'?`)
      }
    }

    if (!isArgs) {
      consola.box(chalk.bold.blue(`RC Generation CLI Experience (rc v${packageJSON.version})`))
      command.help()
    }

    process.exit(0)
  })

rc
  .command('init')
  .description(chalk.bold.gray('⚡ Welcome to the Simple REST Client CLI'))
  .argument('[URL]', 'The URL of the API you want to interact with')
  .option('-m, --method <method>', 'HTTP method to use', 'GET')
  .action(() => {
    consola.info('init command')
  })
rc.parse()
