module.exports = {
  docs: {
    Introduction: ['welcome', 'whatis'],
    Basics: [
      'getting-started',
      'application',
      'server',
      'extractors',
      'handlers',
    ],
    Advanced: [
      'errors',
      'url-dispatch',
      'request',
      'response',
      'testing',
      'middleware',
      'static-files',
    ],
    Protocols: ['websockets', 'http2'],
    Patterns: ['autoreload', 'databases'],
    Diagrams: ['http_server_init', 'conn_lifecycle'],
    'API Documentation': [
      {
        type: 'link',
        label: 'ntex',
        href: 'https://docs.rs/ntex/latest/ntex/',
      },
    ],
  },
};
