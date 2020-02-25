// This file based on https://github.com/microsoft/monaco-languages/blob/209730c94fa2e37c3681f3d40de474853868fd74/src/scheme/scheme.ts

import * as monaco from 'monaco-editor';

import IRichLanguageConfiguration = monaco.languages.LanguageConfiguration;
import ILanguage = monaco.languages.IMonarchLanguage;

export const conf: IRichLanguageConfiguration = {
	comments: {
		lineComment: ';',
		//blockComment: ['#|', '|#'],
	},

	brackets: [['(', ')'], ['{', '}'], ['[', ']']],

	autoClosingPairs: [
		{ open: '{', close: '}' },
		{ open: '[', close: ']' },
		{ open: '(', close: ')' },
		{ open: '"', close: '"' },
	],

	surroundingPairs: [
		{ open: '{', close: '}' },
		{ open: '[', close: ']' },
		{ open: '(', close: ')' },
		{ open: '"', close: '"' },
	],
};

export const language = <ILanguage>{
	defaultToken: '',
	ignoreCase: true,
	tokenPostfix: '.scheme',

	brackets: [
		{ open: '(', close: ')', token: 'delimiter.parenthesis' },
		{ open: '{', close: '}', token: 'delimiter.curly' },
		{ open: '[', close: ']', token: 'delimiter.square' },
	],

	keywords: [
    'quote',
    'do',
    'let',
    'if',
    'lambda',
    'set',
		'vec',
	],

	constants: ['true', 'false'],

	operators: ['eq?', 'eqv?', 'equal?', 'and', 'or', 'not', 'null?'], // TODO

	tokenizer: {
		root: [
			[/#[xXoObB][0-9a-fA-F]+/, 'number.hex'],
			[/[+-]?\d+(?:(?:\.\d*)?(?:[eE][+-]?\d+)?)?/, 'number.float'],

			[
				/(?:\b(?:(define|define-syntax|define-macro))\b)(\s+)((?:\w|\-|\!|\?)*)/,
				['keyword', 'white', 'variable'],
			],

			{ include: '@whitespace' },
			{ include: '@strings' },

			[
				/[a-zA-Z_#][a-zA-Z0-9_\-\?\!\*]*/,
				{
					cases: {
						'@keywords': 'keyword',
						'@constants': 'constant',
						'@operators': 'operators',
						'@default': 'identifier',
					},
				},
			],
		],

		comment: [
			[/[^\|#]+/, 'comment'],
			[/#\|/, 'comment', '@push'],
			[/\|#/, 'comment', '@pop'],
			[/[\|#]/, 'comment'],
		],

		whitespace: [
			[/[ \t\r\n]+/, 'white'],
			[/#\|/, 'comment', '@comment'],
			[/;.*$/, 'comment'],
		],

		strings: [
			[/"$/, 'string', '@popall'],
			[/"(?=.)/, 'string', '@multiLineString'],
		],

		multiLineString: [
			[/[^\\"]+$/, 'string', '@popall'],
			[/[^\\"]+/, 'string'],
			[/\\./, 'string.escape'],
			[/"/, 'string', '@popall'],
			[/\\$/, 'string']
		],
	},
};
