const path = require("path");

const programDir = path.join(__dirname, "..", "programs");
function getProgram(dir, programName) {
  return path.join(programDir, dir, "target", "deploy", programName);
}

module.exports = {
  validator: {
    commitment: "processed",
    programs: [
      {
        label: "Gavel",
        programId: "MyProgram1111111111111111111111111111111111",
        deployPath: getProgram("my-project-name", "mpl_my_project_name.so"),
      },
    ],
  },
};
