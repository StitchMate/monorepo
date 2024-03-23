import { SMUserCard } from "./index.component";

export * from "./index.component";
export default SMUserCard;

customElements.define("sm-user-card", SMUserCard);

declare global {
    interface HTMLElementTagNameMap {
      'sm-user-card': SMUserCard;
    }
  }