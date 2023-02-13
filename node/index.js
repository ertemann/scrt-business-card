import { SecretNetworkClient, Wallet } from "secretjs";
import * as fs from "fs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet(process.env.MNEMONIC);

const contract_wasm = fs.readFileSync(
  "../contract/target/wasm32-unknown-unknown/release/secret_business_card_demo.wasm"
);

let codeId = 19826;
let contractCodeHash =
  "a9664743fb01a8ee2df29f39e382a2e70e87f40dfe33cd40edbeea34deb97835";
let contractAddress = "secret1auclmxkpw28g8qpxe57gqt4t2hgmuuuapa6sta";

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-2",
  url: "https://api.pulsar.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

let upload_contract = async () => {
  let tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: contract_wasm,
      source: "",
      builder: "",
    },
    {
      gasLimit: 4_000_000,
    }
  );

  const codeId = Number(
    tx.arrayLog.find((log) => log.type === "message" && log.key === "code_id")
      .value
  );

  console.log("codeId: ", codeId);

  const contractCodeHash = (
    await secretjs.query.compute.codeHashByCodeId({ code_id: codeId })
  ).code_hash;
  console.log(`Contract hash: ${contractCodeHash}`);
};

// upload_contract();

let instantiate_contract = async () => {
  // Create an instance of the Counter contract, providing a starting count
  const initMsg = { entropy: "this is my entropy, dude!" };
  let tx = await secretjs.tx.compute.instantiateContract(
    {
      code_id: codeId,
      sender: wallet.address,
      code_hash: contractCodeHash,
      init_msg: initMsg,
      label: "Secret Business Card Demo" + Math.ceil(Math.random() * 10000),
    },
    {
      gasLimit: 400_000,
    }
  );

  //Find the contract_address in the logs
  const contractAddress = tx.arrayLog.find(
    (log) => log.type === "message" && log.key === "contract_address"
  ).value;

  console.log(contractAddress);
};

// instantiate_contract();

let createCard = async () => {
  const card_creation_tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        create: {
          card: { name: "card 0", address: "0", phone: "123456789" },
          index: 0,
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(card_creation_tx);
};
// createCard();

let createViewingKey = async () => {
  let viewing_key_creation = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        generate_viewing_key: {
          index: 0,
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(
    viewing_key_creation.arrayLog.find(
      (log) => log.type === "wasm" && log.key === "viewing_key"
    ).value
  );
};
// createViewingKey();

let queryCard = async () => {
  let business_card_query_tx = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_card: {
        wallet: wallet.address,
        viewing_key: viewing_key,
        index: 0,
      },
    },
    code_hash: contractCodeHash,
  });

  console.log(business_card_query_tx);
};
// queryCard();
