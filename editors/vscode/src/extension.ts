import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind,
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
    // Get server path from configuration or use default
    const config = vscode.workspace.getConfiguration('wadescript');
    let serverPath = config.get<string>('serverPath');

    if (!serverPath) {
        // Try to find wadescript in PATH or use a default location
        serverPath = 'wadescript';
    }

    // Server options - run wadescript lsp
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: ['lsp'],
        transport: TransportKind.stdio,
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'wadescript' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.ws'),
        },
    };

    // Create the language client
    client = new LanguageClient(
        'wadescript',
        'WadeScript Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client (and server)
    client.start();

    console.log('WadeScript extension activated');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
