import * as monaco from 'monaco-editor';
import * as kanteraScript from './kanteraScript';
import langSpecs from './langSpecs.md';

const languageId = 'kanteraScript';
monaco.languages.register({
  id: languageId,
  extensions: ['.kntr'],
  aliases: ['kanteraScript', 'kantera script']
});

function createSymbolProposals<T>(range: T) {
  const symbolProposals = Object.entries({
    set: "[Special form] Assignment",
    quote: "[Special form] Quote",
    lambda: "[Special form] Make a lambda function",
    if: "[Special form]",
    defmacro: "[Macro]",
    with_cache: "[Macro]",
    vec: "[Function] Make a vector",
    rgb: "[Function]",
    rgba: "[Function]",
    import_audio: "[Function]",
    import_video: "[Function]",
    import_ttf: "[Function]",
    test_audio: "[Function]",
    audio: "[System variable]",
    video: "[System variable]",
    framerate: "[System variable]",
    samplerate: "[System variable]",
    frame_num: "[System variable]",
    frame_size: "[System variable]",
  }).map(([name, doc]) => ({
      label: name,
      kind: monaco.languages.CompletionItemKind.Function,
      documentation: doc,
      insertText: name,
      range: range
    }));

  const snipetProposals = [
    {
      label: 'red',
      documentation: "A color",
      insertText: '(rgb 1.0 0.0 0.0)'
    },
    {
      label: 'yellow',
      documentation: "A color",
      insertText: '(rgb 1.0 1.0 0.0)'
    },
    {
      label: 'blue',
      documentation: "A color",
      insertText: '(rgb 0.0 0.0 1.0)'
    },
    {
      label: 'white',
      documentation: "A color",
      insertText: '(rgb 1.0 1.0 1.0)'
    },
    {
      label: 'black',
      documentation: "A color",
      insertText: '(rgb 0.0 0.0 0.0)'
    },
    {
      label: 'transparent',
      documentation: "A color",
      insertText: '(rgba 0.0 0.0 0.0 0.0)'
    },
  ].map(obj => ({
    kind: monaco.languages.CompletionItemKind.Snippet,
    insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
    range: range,
    ...obj
  }));

  return [...symbolProposals, ...snipetProposals];
}

function createFormProposals<T>(range: T) {
  return [
    {
      label: 'set video',
      documentation: "Set as master video render",
      insertText: 'set video ${1:my_video_render}'
    },
    {
      label: 'set audio',
      documentation: "Set as master audio render",
      insertText: 'set audio ${1:my_audio_render}'
    },
    {
      label: 'rgb',
      documentation: "Make RGBA color from RGB",
      insertText: 'rgb ${1:1.0} ${2:1.0} ${3:1.0}'
    },
    {
      label: 'rgba',
      documentation: "Make RGBA color",
      insertText: 'rgba ${1:1.0} ${2:1.0} ${3:1.0} ${4:1.0}'
    },
    {
      label: 'plain',
      documentation: "render 'Plain'",
      insertText: 'plain ${1:color}'
    },
    {
      label: 'text_to_image',
      documentation: "TBD",
      insertText: 'text_to_image "${1:my_text}" ${2:font_size} ${3:font?}'
    },
    {
      label: 'image_render',
      documentation: "TBD",
      insertText: 'image_render ${1:image} ${2:bg_color}'
    },
    {
      label: 'composite',
      documentation: "TBD",
      insertText: 'composite (vec ${1:render} ${2:\'normal})\n${3:...}'
    },
  ].map(obj => ({
    kind: monaco.languages.CompletionItemKind.Snippet,
    insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
    range: range,
    ...obj
  }));
}

monaco.languages.registerCompletionItemProvider(languageId, {
  provideCompletionItems: function(model, position) {
    const textUntilPosition = model.getValueInRange({startLineNumber: 1, startColumn: 1, endLineNumber: position.lineNumber, endColumn: position.column});
    const suggestions = [];

    if (true) {
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn
      };
      suggestions.push(...createSymbolProposals(range));
    }

    //if (textUntilPosition.match(/\(\s*?([^"()]*)?$/)) {
    if (textUntilPosition.match(/\(.*?$/)) {
      const openParenRange = (model.findPreviousMatch("(", position, false, false, null, false) as monaco.editor.FindMatch).range;
      const closeParenRange = model.findNextMatch(")", position, false, false, null, false)?.range;
      const word = model.getWordUntilPosition(position);
      const range = {
        startLineNumber: openParenRange.startLineNumber,
        endLineNumber: Math.max(closeParenRange?.endLineNumber || 0, position.lineNumber),
        startColumn: openParenRange.startColumn + 1,
        endColumn: closeParenRange?.endColumn ? closeParenRange?.endColumn - 1 : word.endColumn
      };
      suggestions.push(...createFormProposals(range));
    }

    return { suggestions };
  }
});

// TODO: Refactor me!
const symbolDescriptions: {[key: string]: string} = {};
const ls = langSpecs.split(/^# (.+)$/m);
const a: {[key: string]: string} = {};
for (let i = 1; ls[i]; i += 2) {
  a[ls[i]] = ls[i + 1];
}
if (a['symbolDescriptions']) {
  const ls = a['symbolDescriptions'].split(/^## (.+)$/m);
  for (let i = 1; ls[i]; i += 2) {
    symbolDescriptions[ls[i]] = ls[i + 1];
  }
}

monaco.languages.registerHoverProvider(languageId, {
	provideHover(model, position) {
    const word = model.getWordAtPosition(position);
    if (word && symbolDescriptions[word.word]) {
      return {
        range: new monaco.Range(
          position.lineNumber,
          word.startColumn,
          position.lineNumber,
          word.endColumn),
        contents: [
          { value: `**${word.word}**` },
          { value: symbolDescriptions[word.word] }
        ]
      };
    } else {
      return null;
    }
	}
});

monaco.languages.setMonarchTokensProvider(languageId, kanteraScript.language);

monaco.languages.onLanguage(languageId, () => {
  monaco.languages.setLanguageConfiguration(languageId, kanteraScript.conf);
});
