const Elements = ( e => {

    let pageTitle = e => {
        let pageTitle = document.title.split(':');
        document.getElementById(Elements.headerLabel).textContent = `${ pageTitle[1] }: ${ pageTitle[2] }`;
    };

    return {
        refsList: 'refsList',
        bibTexCode: 'bibTextCode',
        headerLabel: 'headerLabel',
        bibTexModal: 'bibTextModal',
        docsSourceList: 'docsSourceList',
        scrollToTopBtn: 'scrollToTopBtn',
        scrollToRefsBtn: 'scrollToRefsBtn',
        scrollToDocsBtn: 'scrollToDocsBtn',
        bibTextMaskModal: 'bibTextMaskModal',

        classNames: {
            modal: 'modal',
            refsList: 'refs-list',
            markdownBody: 'markdown-body',
            contentModal: 'modal-content',
            pluginSection: 'plugin-section',
            docsSourceList: 'docs-source-list',
        },

        pageTitle: () => { return pageTitle(); },
    };

})();
