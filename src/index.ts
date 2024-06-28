import process from 'node:process'
import { Command } from 'commander'
import consola from 'consola'
import chalk from 'chalk'
import packageJSON from '../package.json' assert { type: 'json' }
import { findMostMatchText } from './helpers/diff'
import { methodDescriptions } from './constants/messages'

const rc = new Command()

export type CommandName = 'init' | 'get' | 'post' | 'put' | 'patch' | 'delete'

const commandList: CommandName[] = ['init', 'get', 'post', 'put', 'patch', 'delete']

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
  .description(chalk.bold.gray('Initialize a new REST Client'))
  .argument('[BASE_URL]', 'The base URL of the API you want to interact with')
  .option('-m, --method <method>', 'HTTP method to use', 'GET')
  .action(() => {
    consola.info('init command')
  })

rc
  .command('get')
  .description(chalk.bold.gray('Make a GET request to the API'))
  .argument(methodDescriptions)
  .option('-i, --id <id>', 'The ID of the resource you want to get')
  .option('-q, --query <query>', 'Query parameters to send with the request')
  .option('-h, --header <header>', 'Headers to send with the request')
  .action(() => {
    consola.info('get command')
  })

rc
  .command('post')
  .description(chalk.bold.gray('Make a POST request to the API'))
  .argument(methodDescriptions)
  .option('-b, --body <body>', 'The body of the request')
  .option('-h, --header <header>', 'Headers to send with the request')
  .action(() => {
    consola.info('post command')
  })

rc
  .command('put')
  .description(chalk.bold.gray('Make a PUT request to the API'))
  .argument(methodDescriptions)
  .option('-i, --id <id>', 'The ID of the resource you want to update')
  .option('-b, --body <body>', 'The body of the request')
  .option('-h, --header <header>', 'Headers to send with the request')
  .action(() => {
    consola.info('put command')
  })

rc
  .command('patch')
  .description(chalk.bold.gray('Make a PATCH request to the API'))
  .argument(methodDescriptions)
  .option('-i, --id <id>', 'The ID of the resource you want to update')
  .option('-b, --body <body>', 'The body of the request')
  .option('-h, --header <header>', 'Headers to send with the request')
  .action(() => {
    consola.info('patch command')
  })

rc
  .command('delete')
  .description(chalk.bold.gray('Make a DELETE request to the API'))
  .argument(methodDescriptions)
  .option('-i, --id <id>', 'The ID of the resource you want to delete')
  .option('-h, --header <header>', 'Headers to send with the request')
  .action(() => {
    consola.info('delete command')
  })

rc.parse()
