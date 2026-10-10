const path = require('path');

const repo = process.env.GITHUB_REPOSITORY || 'ntex-rs/website';
const [organizationName = 'ntex-rs', projectName = 'website'] = repo.split('/');

module.exports = {
  title: 'ntex',
  tagline: 'ntex is a powerful, pragmatic, and extremely fast framework for composable networking services for Rust',
  url: process.env.DOCUSAURUS_URL || 'https://ntex.rs',
  baseUrl: process.env.DOCUSAURUS_BASE_URL || '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/ntex-icon.png',
  organizationName: process.env.DOCUSAURUS_ORGANIZATION || organizationName,
  projectName: process.env.DOCUSAURUS_PROJECT || projectName,
  themeConfig: {
    navbar: {
      title: 'ntex',
      logo: {
        alt: 'ntex logo',
        src: 'img/ntex-icon.svg',
      },
      items: [
        {
          to: 'docs',
          activeBasePath: 'docs',
          label: 'Documentation',
          position: 'left',
        },
        {
          to: 'community',
          activeBasePath: 'community',
          label: 'Community',
          position: 'left',
        },
        {
          to: 'code',
          activeBasePath: 'code',
          label: 'Code',
          position: 'left',
        },
      ],
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} The ntex contributors`,
    },
    prism: {
      // dracula is closest to docs.rs, where keywords are highlighted
      theme: require('prism-react-renderer').themes.okaidia,
      additionalLanguages: ['rust', 'toml'],
      defaultLanguage: 'rust'
    }
  },
  plugins: ["docusaurus-plugin-sass"],
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl:
            'https://github.com/ntex-rs/website/edit/master/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
