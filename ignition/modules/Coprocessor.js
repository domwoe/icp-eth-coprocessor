const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

const CoprocessorModule = buildModule("CoprocessorModule", (m) => {
  const coprocessor = m.contract("Coprocessor", ["0xCFa17195BfD87CDE897392f01ebd8450a28243d7"]);

  return { coprocessor };
});

module.exports = CoprocessorModule;