export function create(name: string) {
    customElements.define(
        name,
        <CustomElementConstructor>class extends HTMLElement {
            constructor() {
                super();
                // const name = this.nodeName.toLowerCase();
                let template = document.getElementById(name) as HTMLTemplateElement | null;

                if (!template) {
                    throw new Error(`Could not find template for ${name}`)
                }

                let templateContent = template.content;

                const shadowRoot = this.attachShadow({ mode: 'open' });
                shadowRoot.appendChild(templateContent.cloneNode(true));
            }
        }
    );
}