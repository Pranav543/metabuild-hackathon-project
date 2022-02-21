## To build smart contracts

```bash
cd contracts
```

```bash
./build.sh
```

## To deploy smart contracts

```bash
. ./deploy.sh
```

> additional dot is required for running shell script in current session, so that all required parameters will be initialized correctly

## Get Some ELEMENT Token in your Near wallet

```bash
ID=<YOUR_WALLET_ID>
echo $ID
```

```bash
near call $element storage_deposit '' --accountId $ID --amount 0.00125
```

```bash
near call $element ft_transfer '{"receiver_id": "'$ID'", "amount": "30"}' --accountId $element --amount 0.000000000000000000000001
```

Now get dev account id from .env file and replace that address with element-app/.env account id

## Try out frontend

```bash
cd .. && cd element-app
```

```bash
npm install
```

```bash
npm run develop
```
