<script setup lang="ts">
    import { Avatar } from "@/components";
    import { useAccountStore } from "@/modules";
    import { ref } from "vue";

    const modal = ref<HTMLDialogElement>();
    const accountStore = useAccountStore();

    const openModal = () => {
        modal.value?.showModal();
    };
</script>

<template>
    <div class="w-full h-full p-6">
        <div class="card w-full bg-base-100">
            <div class="card-body p-2! flex flex-row">
                <button class="btn btn-sm btn-soft btn-ghost" @click="openModal">
                    <i class="icon-[material-symbols--add-circle-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Account.TopButtons.AddAccount") }}</span>
                </button>
                <button class="btn btn-sm btn-soft btn-ghost">
                    <i class="icon-[material-symbols--drive-file-move-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Account.TopButtons.ImportAccount") }}</span>
                </button>
                <button class="btn btn-sm btn-soft btn-ghost">
                    <i class="icon-[material-symbols--drive-file-move-rtl-outline-rounded] size-5 -ml-1 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Account.TopButtons.ExportAccount") }}</span>
                </button>
                <button
                    class="btn btn-sm btn-soft btn-ghost btn-accent ml-auto"
                    @click="'https://www.xbox.com/games/store/minecraft-java-bedrock-edition-for-pc/9nxp44l49shj'">
                    <i class="icon-[material-symbols--shopping-cart-checkout-outline-rounded] size-5 -mx-1"></i>
                </button>
            </div>
        </div>
        <div class="card w-full bg-base-100 mt-2">
            <div class="card-body p-2! flex flex-col">
                <div class="overflow-x-auto">
                    <table class="table">
                        <thead>
                            <tr>
                                <th class="w-0"></th>
                                <th class="w-12"></th>
                                <th class="w-1/2">{{ $t("Main.r/Account.TableHeader.Name") }}</th>
                                <th class="w-1/2">{{ $t("Main.r/Account.TableHeader.LoginMethod") }}</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <th>
                                    <div class="inline-grid *:[grid-area:1/1] -translate-y-1">
                                        <div class="status status-success animate-ping"></div>
                                        <div class="status status-success"></div>
                                    </div>
                                </th>
                                <td>
                                    <Avatar :name="accountStore.AccountName" extra-class="w-8" />
                                </td>
                                <td>LingyunAwA</td>
                                <td>Microsoft</td>
                                <td class="flex justify-end gap-2">
                                    <button class="btn btn-sm btn-soft btn-success aspect-square">
                                        <i class="icon-[material-symbols--motion-play-outline-rounded] size-5 -mx-2"></i>
                                    </button>
                                    <button class="btn btn-sm btn-soft btn-info aspect-square" disabled>
                                        <i class="icon-[material-symbols--visibility-outline-rounded] size-5 -mx-2"></i>
                                    </button>
                                    <button class="btn btn-sm btn-soft btn-warning aspect-square" disabled>
                                        <i
                                            class="icon-[material-symbols--edit-document-outline-rounded] size-5 -mx-2"></i>
                                    </button>
                                    <button class="btn btn-sm btn-soft btn-error aspect-square">
                                        <i class="icon-[material-symbols--delete-outline-rounded] size-5 -mx-2"></i>
                                    </button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>

        <dialog ref="modal" class="modal">
            <div class="modal-box flex flex-col items-center gap-2">
                <h3 class="text-xl font-bold">{{ $t("Main.r/Account.Modal.Add.Title") }}</h3>
                <p class="text-sm opacity-50">{{ $t("Main.r/Account.Modal.Add.Description") }}</p>
                <div class="divider w-76 mx-auto my-0"></div>
                <button class="btn bg-[#2F2F2F] text-white border-black w-66">
                    <svg
                        aria-label="Microsoft logo"
                        width="24"
                        height="24"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 512 512">
                        <path d="M96 96H247V247H96" fill="#f24f23"></path>
                        <path d="M265 96V247H416V96" fill="#7eba03"></path>
                        <path d="M96 265H247V416H96" fill="#3ca4ef"></path>
                        <path d="M265 265H416V416H265" fill="#f9ba00"></path>
                    </svg>
                    <span class="text-sm">{{ $t("Main.r/Account.Modal.Add.MsLogin") }}</span>
                </button>
                <button class="btn w-66">
                    <i class="icon-[material-symbols--safety-check-off-outline-rounded] size-6 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Account.Modal.Add.Offline") }}</span>
                </button>
                <button class="btn w-66">
                    <i class="icon-[material-symbols--assured-workload-outline-rounded] size-6 mr-1"></i>
                    <span class="text-sm">{{ $t("Main.r/Account.Modal.Add.TrdParty") }}</span>
                </button>
                <div class="divider w-76 mx-auto my-0"></div>
                <form method="dialog" class="w-66">
                    <button class="btn w-full">{{ $t("Main.r/Account.Modal.Add.Cancel") }}</button>
                </form>
            </div>
        </dialog>
    </div>
</template>

<style lang="scss" scoped>
    tbody > tr > :nth-child(2) {
        display: flex;
        justify-content: center;
        align-items: center;
    }
</style>
