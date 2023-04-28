const path = require("path");
const { generateIdl } = require("@metaplex-foundation/shank-js");

const idlDir = path.join(__dirname, "..", "idls");
const binaryInstallDir = path.join(__dirname, "..", ".crates");
const programDir = path.join(__dirname, "..", "programs");

generateIdl({
  generator: "shank",
  programName: "my_project_name",
  programId: "MyProgram1111111111111111111111111111111111",
  idlDir,
  binaryInstallDir,
  programDir: path.join(programDir, "my-project-name"),
});
