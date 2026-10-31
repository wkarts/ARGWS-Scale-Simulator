<script setup lang="ts">
import BasePage from "../components/base/BasePage.vue";
import BaseSectionCard from "../components/base/BaseSectionCard.vue";
</script>
<template>
  <BasePage title="Guia de simulação" subtitle="Do primeiro quadro ao teste de integração.">
    <div class="grid columns-2 mobile-columns-1">
      <BaseSectionCard title="1. Escolha o indicador"><p>Abra Simulador de balanças e selecione um dos nove modelos. Confira a prévia do quadro e os parâmetros de capacidade, divisão e peso inicial.</p></BaseSectionCard>
      <BaseSectionCard title="2. Configure a conexão"><p>Para TCP, escolha endereço e porta livres e conecte seu sistema cliente. Para RS232, escolha a porta serial e a velocidade compatível com o sistema que receberá os dados.</p><p>Uma conexão serial virtual precisa de um par de portas configurado no sistema operacional. Não conecte os dois aplicativos à mesma ponta.</p></BaseSectionCard>
      <BaseSectionCard title="3. Inicie e acompanhe"><p>Inicie a instância e observe o peso, os clientes conectados, o último quadro e os logs. Ajuste o peso, aplique tara ou simule instabilidade pelos controles disponíveis.</p><p>No modo por comando, a balança responde às solicitações do cliente. No modo contínuo, envia os quadros no intervalo configurado.</p></BaseSectionCard>
      <BaseSectionCard title="4. Confira o protocolo"><p>Ajuste bits de dados, paridade e bits de parada conforme o receptor. O Toledo 8540 inicia em 7E1 e permite 7E2; os demais modelos iniciam em 8N1. Ao trocar o modelo ou preset, esses padrões são restaurados. Os quadros contínuos contêm bytes de status; a visualização textual pode incluir caracteres não imprimíveis.</p><p>Ao terminar, pare as instâncias para liberar as portas. A prévia web não abre portas TCP ou RS232.</p></BaseSectionCard>
      <BaseSectionCard title="Testes avançados de enquadramento"><p>A seção Enquadramento avançado é opcional e fica desativada por padrão. Permite substituir ou remover prefixo e terminador em HEX, por exemplo STX 02, ETX 03, CR 0D e LF 0A.</p><p>No Toledo 8540, o checksum pode ser correto, corrompido ou omitido para testar o receptor. Essas opções podem produzir quadros inválidos. Confira a prévia; trocar o driver ou preset restaura o enquadramento original.</p></BaseSectionCard>
      <BaseSectionCard title="Acesso e diagnóstico"><p>As contas, empresas e permissões ficam em Administração. Configurações, banco local e diagnósticos ficam em Ferramentas. O tema claro é o padrão; sua escolha de tema é salva localmente.</p></BaseSectionCard>
    </div>
  </BasePage>
</template>
