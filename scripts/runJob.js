const hre = require("hardhat");

async function main() {
  try {
  
    const coprocessor = await hre.ethers.getContractFactory("Coprocessor");

    // Connect to the deployed contract
    const contractAddress = "0xFe15805f952c6A1a465aDdD993457Ec640Ee57aA";
    const contract = await coprocessor.attach(contractAddress);


    const res = await contract.newJob();

    console.log(res);

  } catch (error) {
    console.error(error);
    process.exit(1);
  }
}

main();