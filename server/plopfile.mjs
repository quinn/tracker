export default function (plop) {
	plop.setGenerator('route', {
		description: 'this is a skeleton plopfile',
		prompts: [
			{
				type: 'input',
				name: 'name',
				message: 'name of route',
			}
		],
		actions: [
			{
				type: 'add',
				path: 'src/routes/{{name}}_route.rs',
				templateFile: 'templates/route.rs.hbs',
			},
			{
				type: 'modify',
				path: 'src/routes/mod.rs',
				pattern: /(mod .*)/gi,
				template: '$1\nmod {{name}}_route;',
			},
			{
				type: 'modify',
				path: 'src/routes/mod.rs',
				pattern: /(pub use .*)/gi,
				template: '$1\npub use {{name}}_route::{{name}};',
			},
		]
	})
}
