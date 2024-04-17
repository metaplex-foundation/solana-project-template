const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "mpl_project_name_program.json")]);

// Update programs.
kinobi.update(
  new k.updateProgramsVisitor({
    mplProjectNameProgram: { name: "mplProjectName" },
  })
);

// Update accounts.
kinobi.update(
  new k.updateAccountsVisitor({
    myPdaAccount: {
      seeds: [
        k.constantPdaSeedNodeFromString("myPdaAccount"),
        k.programIdPdaSeedNode(),
        k.variablePdaSeedNode("authority", k.publicKeyTypeNode(), "The address of the authority"),
        k.variablePdaSeedNode("name", k.stringTypeNode(), "The name of the account"),
      ],
    },
  })
);

// Update instructions.
kinobi.update(
  new k.updateInstructionsVisitor({
    create: {
      byteDeltas: [
        k.instructionByteDeltaNode(k.accountLinkNode("myAccount")),
      ],
    },
  })
);

// Set ShankAccount discriminator.
const key = (name) => ({ field: "key", value: k.enumValueNode("Key", name) });
kinobi.update(
  new k.setAccountDiscriminatorFromFieldVisitor({
    myAccount: key("MyAccount"),
    myPdaAccount: key("MyPdaAccount"),
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.renderJavaScriptVisitor(jsDir, { prettier }));

// Render Rust.
const crateDir = path.join(clientDir, "rust");
const rustDir = path.join(clientDir, "rust", "src", "generated");
kinobi.accept(
  new k.renderRustVisitor(rustDir, {
    formatCode: true,
    crateFolder: crateDir,
  })
);
