const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "mpl_project_name.json")]);

// Update accounts.
kinobi.update(
  new k.UpdateAccountsVisitor({
    // ...
  })
);

// Update instructions.
kinobi.update(
  new k.UpdateInstructionsVisitor({
    // ...
  })
);

// Set ShankAccount discriminator.
kinobi.update(
  new k.SetAccountDiscriminatorFromFieldVisitor({
    myAccount: { field: "key", value: k.vEnum("Key", "MyAccount") },
  })
);

// Render JavaScript.
const jsDir = path.join(clientDir, "js", "src", "generated");
const prettier = require(path.join(clientDir, "js", ".prettierrc.json"));
kinobi.accept(new k.RenderJavaScriptVisitor(jsDir, { prettier }));
