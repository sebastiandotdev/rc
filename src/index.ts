import process from 'node:process'
import { Command } from 'commander'
import chalk from 'chalk'
import { intro, log } from '@clack/prompts'
import packageJSON from '../package.json' assert { type: 'json' }
import { findMostMatchText } from './helpers/diff'
import { descriptionRC, helpDescription, methodDescriptions } from './constants/messages'
import { bodyOption, headerOption, helpOption, idOption, queryOption } from './constants/options'
import { initAction } from './actions'

const rc = new Command()

export type CommandName = 'init' | 'get' | 'post' | 'put' | 'patch' | 'delete'

const commandList: CommandName[] = ['init', 'get', 'post', 'put', 'patch', 'delete']

rc
  .name('rc')
  .usage('<command> [options]')
  .version(packageJSON.version, '-v, --version', 'output the current version')
  .helpOption(helpOption, helpDescription)
  .allowUnknownOption()
  .action((_, command) => {
    intro(chalk.bold.blue(`RC Generation CLI Experience (rc v${packageJSON.version})`))
    log.message(chalk.bold.gray(descriptionRC))
    let isArgs = false

    if (command) {
      const args = command.args?.[0]

      if (args && !commandList.includes(args as CommandName)) {
        isArgs = true
        const matchCommand = findMostMatchText(commandList, args)

        if (!matchCommand) {
          log.error(`The '${args}' command is not available in RC Unknown command `)
          return
        }

        log.info(`Unknown command '${args}', Did you mean '${chalk.underline(matchCommand)}'?`)
      }
    }

    if (!isArgs) {
      log.info('Please run `rc --help` to see available commands')
    }

    process.exit(0)
  })

rc
  .command('init')
  .description(chalk.bold.gray('Initialize a new REST Client'))
  .helpOption(helpOption, helpDescription)
  .action(initAction)

rc
  .command('get')
  .description(chalk.bold.gray('Make a GET request to the API'))
  .argument(methodDescriptions)
  .helpOption(helpOption, helpDescription)
  .option(idOption, 'The ID of the resource you want to get')
  .option(queryOption, 'Query parameters to send with the request')
  .option(headerOption, headerOption)

rc
  .command('post')
  .description(chalk.bold.gray('Make a POST request to the API'))
  .argument(methodDescriptions)
  .helpOption(helpOption, helpDescription)
  .option(bodyOption, 'The body of the request')
  .option(headerOption, 'Headers to send with the request')

rc
  .command('put')
  .description(chalk.bold.gray('Make a PUT request to the API'))
  .argument(methodDescriptions)
  .helpOption(helpOption, helpDescription)
  .option(idOption, 'The ID of the resource you want to update')
  .option(bodyOption, 'The body of the request')
  .option(headerOption, headerOption)

rc
  .command('patch')
  .description(chalk.bold.gray('Make a PATCH request to the API'))
  .argument(methodDescriptions)
  .helpOption(helpOption, helpDescription)
  .option(idOption, 'The ID of the resource you want to update')
  .option(bodyOption, 'The body of the request')
  .option(headerOption, headerOption)

rc
  .command('delete')
  .description(chalk.bold.gray('Make a DELETE request to the API'))
  .argument(methodDescriptions)
  .helpOption(helpOption, helpDescription)
  .option(idOption, 'The ID of the resource you want to delete')
  .option(headerOption, headerOption)

rc.parse()
