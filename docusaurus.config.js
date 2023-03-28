const path = require('path');

module.exports = {
  title: 'Ntex',
  tagline: 'Ntex is a powerful, pragmatic, and extremely fast framework for composable networking services for Rust',
  url: 'https://ntex.rs',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/logo.png',
  organizationName: 'ntex', // Usually your GitHub org/user name.
  projectName: 'ntex', // Usually your repo name.
  themeConfig: {
    navbar: {
      title: 'Ntex',
      logo: {
        alt: 'Ntex Logo',
        src: 'img/logo.png',
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
      copyright: `Copyright © ${new Date().getFullYear()} The Ntex Contributors`,
    },
    prism: {
      // dracula is closest to docs.rs, where keywords are highlighted
      theme: require('prism-react-renderer/themes/okaidia'),
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
            'https://github.com/ntex-rs/ntex-website/edit/master/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
