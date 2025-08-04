import * as monaco from "monaco-editor";

monaco.languages.register({ id: "tundra" });

monaco.languages.setMonarchTokensProvider("tundra", {
  tokenizer: {
    root: [
      [/[-+]?\d+(\.\d+)?/, "number"],          // ints / floats
      [/"[^"]*"/, "string"],                   // double-quoted strings
      [/'[^']*'/, "string"],                   // single-quoted strings
      [/\b(var|fun|class|for|while|if|else|return|print)\b/, "keyword"],
      [/[\+\-\*\/%]|\*\*|==|!=|<=|>=|<|>/, "operator"],
      [/[{}\(\)\[\]:]/, "@brackets"],          
      [/[a-zA-Z_]\w*/, "identifier"],
      [/#.*$/, "comment"],
    ],
  },
});

monaco.languages.setLanguageConfiguration("tundra", {
  indentationRules: {

    increaseIndentPattern: /^\s*(if|while|for|fun|class|else)\b.*:\s*$/,

    decreaseIndentPattern: /^\s*(else)\b.*$/,
  },
  onEnterRules: [
    {
 
      beforeText: /^\s*(if|while|for|fun|class|else)\b.*:\s*$/,
      action: {
        indentAction: monaco.languages.IndentAction.Indent,
      },
    },
    {

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


monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
  noSemanticValidation: true,
  noSyntaxValidation: true,
});