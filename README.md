## porkbun-rs

Create, delete, and list DNS records for domains on [Porkbun](https://porkbun.com/)

```bash
❯ porkbun-rs --help
Usage: porkbun-rs [OPTIONS] [COMMAND]

Commands:
  create-record  Create a new DNS record for a given domain
  delete-record  Delete a DNS record for a given domain
  list-domains   List all domains associated with the account
  list-records   List all records for a given domain
  help           Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...                   Turn debugging information on
      --generate <GENERATOR>       [possible values: bash, elvish, fish, powershell, zsh]
  -b, --base-url <BASE_URL>        [env: BASE_URL=] [default: https://api.porkbun.com/api/json/]
  -v, --url-version <URL_VERSION>  [env: BASE_URL_VERSION=] [default: v3]
  -a, --api-key <API_KEY>          Project where we need to commit file to [env: API_KEY=]
  -s, --secret-key <SECRET_KEY>    [env: SECRET_KEY=]
  -h, --help                       Print help
  -V, --version                    Print version


```