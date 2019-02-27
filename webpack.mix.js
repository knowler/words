const mix = require('laravel-mix');
const tailwindcss = require('tailwindcss');
const glob = require('glob-all');
const PurgecssPlugin = require('purgecss-webpack-plugin');

class TailwindExtractor {
  static extract(content) {
    return content.match(/[A-Za-z0-9-_:\/]+/g) || [];
  }
}

/*
|-------------------------------------------------------------------------------
| Base configuration
|-------------------------------------------------------------------------------
|
| Set all your postcss and javascript entries here.
|
*/
mix
  .postCss('assets/styles/main.css', 'public/styles', [
    tailwindcss('tailwind.js'),
    require('postcss-nesting')(),
  ])
  .js('assets/scripts/main.js', 'public/scripts');

/*
|-------------------------------------------------------------------------------
| Production configuration
|-------------------------------------------------------------------------------
|
| The following is for production builds.
|
*/
if (mix.inProduction()) {
  mix.webpackConfig({
    plugins: [
      new PurgecssPlugin({
        paths: glob.sync([
          path.join(__dirname, 'templates/**/*.html'),
          path.join(__dirname, 'assets/scripts/**/*.js')
        ]),
        extractors: [
          {
            extractor: TailwindExtractor,
            extensions: ['html', 'js']
          }
        ],
        whitelist: ['pre'],
      })
    ]
  });
} else {
  mix.browserSync('127.0.0.1:1111');
}
