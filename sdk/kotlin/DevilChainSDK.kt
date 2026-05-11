package network.devilchain.sdk

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.net.HttpURLConnection
import java.net.URL

class DevilChainSDK(private val baseUrl: String = "http://localhost:8545") {
    private suspend fun get(path: String): JSONObject = withContext(Dispatchers.IO) {
        val conn = URL("$baseUrl$path").openConnection() as HttpURLConnection
        conn.requestMethod = "GET"; conn.connect()
        JSONObject(conn.inputStream.bufferedReader().readText())
    }
    private suspend fun post(path: String, body: JSONObject): JSONObject = withContext(Dispatchers.IO) {
        val conn = URL("$baseUrl$path").openConnection() as HttpURLConnection
        conn.requestMethod = "POST"; conn.setRequestProperty("Content-Type", "application/json")
        conn.doOutput = true; conn.outputStream.write(body.toString().toByteArray()); conn.connect()
        JSONObject(conn.inputStream.bufferedReader().readText())
    }
    suspend fun getStatus() = get("/api/status")
    suspend fun getLatestBlock() = get("/api/block/latest")
    suspend fun getBlock(h: Int) = get("/api/block/$h")
    suspend fun getWallet(addr: String) = get("/api/wallet/$addr")
    suspend fun getValidators() = get("/api/validators")
    suspend fun getDaoProposals() = get("/api/dao/proposals")
    suspend fun sendTransaction(from: String, to: String, amount: Double, gasFee: Double, sig: String) =
        post("/api/send", JSONObject().apply { put("from",from); put("to",to); put("amount",amount); put("gas_fee",gasFee); put("signature",sig) })
    suspend fun stake(addr: String, amount: Double, sig: String) =
        post("/api/stake", JSONObject().apply { put("address",addr); put("amount",amount); put("signature",sig) })
}
