const vscode = require("vscode");

function activate(context) {
  const startREPL = vscode.commands.registerCommand("veyra.startREPL", () => {
    const terminal = vscode.window.createTerminal({
      name: "Veyra REPL",
    });

    terminal.show();
    terminal.sendText("veyra");
  });

  context.subscriptions.push(startREPL);
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
};
