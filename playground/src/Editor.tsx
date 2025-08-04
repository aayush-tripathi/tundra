// Editor.tsx
import { useCallback } from "react";
import MonacoEditor, { Monaco } from "@monaco-editor/react";

const registerTundra = (monaco: Monaco) => {
/*──────────────── 1.  Register the language ────────────────*/
monaco.languages.register({ id: "tundra" });

/*──────────────── 2.  Tokenizer (Monarch) ───────────────────*/
monaco.languages.setMonarchTokensProvider("tundra", {
  tokenizer: {
    root: [
      [/[-+]?\d+(\.\d+)?/, "number"],          // ints / floats
      [/"[^"]*"/, "string"],                   // double-quoted strings
      [/'[^']*'/, "string"],                   // single-quoted strings
      [/\b(var|fun|class|for|while|if|else|return|print)\b/, "keyword"],
      [/[\+\-\*\/%]|\*\*|==|!=|<=|>=|<|>/, "operator"],
      [/[{}\(\)\[\]:]/, "@brackets"],          // include colon as bracket
      [/[a-zA-Z_]\w*/, "identifier"],
      [/#.*$/, "comment"],
    ],
  },
});

/*──────────────── 3.  Language configuration for indentation ────────────*/
monaco.languages.setLanguageConfiguration("tundra", {
  indentationRules: {
    // Increase indent after lines ending with ':'
    increaseIndentPattern: /^\s*(if|while|for|fun|class|else)\b.*:\s*$/,
    // Decrease indent for lines starting with 'else'
    decreaseIndentPattern: /^\s*(else)\b.*$/,
  },
  onEnterRules: [
    {
      // After a line ending with ':', increase indent
      beforeText: /^\s*(if|while|for|fun|class|else)\b.*:\s*$/,
      action: {
        indentAction: monaco.languages.IndentAction.Indent,
      },
    },
    {
      // For 'else' statements, align with the corresponding 'if'
      beforeText: /^\s*else\s*:\s*$/,
      action: {
        indentAction: monaco.languages.IndentAction.Outdent,
      },
    },
  ],
  brackets: [
    ['(', ')'],
    ['[', ']'],
  ],
});

/*──────────────── 4.  Snippet-style completions ─────────────*/
monaco.languages.registerCompletionItemProvider("tundra", {
  provideCompletionItems: (model, position) => {
    const word = model.getWordUntilPosition(position);
    const range = {
      startLineNumber: position.lineNumber,
      endLineNumber: position.lineNumber,
      startColumn: word.startColumn,
      endColumn: word.endColumn,
    };

    return {
      suggestions: [
        {
          label: "print",
          kind: monaco.languages.CompletionItemKind.Function,
          insertText: 'print(${1:expr})',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "var",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: 'var ${1:name} = ${2:value}',
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "fun",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'fun ${1:name}(${2:params}) :',
            '    ${3:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "class",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'class ${1:Name} :',
            '    ${2:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "for",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'for ${1:i} in ${2:range} :',
            '    ${3:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "while",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'while (${1:condition}) :',
            '    ${2:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "if",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'if (${1:condition}) :',
            '    ${2:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
        {
          label: "else",
          kind: monaco.languages.CompletionItemKind.Keyword,
          insertText: [
            'else :',
            '    ${1:# body}',
          ].join('\n'),
          insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
          range,
        },
      ],
    };
  },
});

/*──────────────── 5.  (Optional) turn off the default JS/TS diagnostics ──*/
monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
  noSemanticValidation: true,
  noSyntaxValidation: true,
});
}

interface Props {
  code: string;
  setCode: (c: string) => void;
}

export const Editor = ({ code, setCode }: Props) => {
  // run once, before the editor mounts
  const handleBeforeMount = useCallback((monaco: Monaco) => {
    registerTundra(monaco);
  }, []);

  return (
    <MonacoEditor
      language="tundra"
      value={code}
      beforeMount={handleBeforeMount}
      onChange={(v) => setCode(v ?? "")}
      options={{
        minimap: { enabled: false },
        fontSize: 14,
        lineNumbers: "on",
        wordWrap: "on",
        automaticLayout: true,
        tabSize: 4,
        insertSpaces: true,
        scrollBeyondLastLine: false,
        folding: true,
        autoIndent: "full",
      }}
      theme="vs-dark"
    />
  );
};
