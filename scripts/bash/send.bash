export $(cat .env.local | xargs)
export $(cat .env.generated | xargs)

npx ts-node ./scripts/ts/send.ts