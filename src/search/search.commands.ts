import {safeInvoke} from "../safeInvoke";
import {IndexedEntry} from "./search.types";

/**
 * Search the in-memory index for entries whose name matches the given query.
 * Backed by the trigram index built at startup over the user directories.
 */
export async function searchIndex(query: string): Promise<IndexedEntry[]> {
    return safeInvoke<IndexedEntry[]>('search_index', {query});
}
